use color_eyre::eyre::Result;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Field, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing, TextOptions},
    tokenizer::{
        AsciiFoldingFilter, LowerCaser, NgramTokenizer, RawTokenizer, TextAnalyzer,
        TokenizerManager,
    },
    Document, Index,
};
use weather_lib::locations::Location;

const MEM_BYTES_PER_THREAD: usize = 3_000_000;

fn ngram_options() -> TextOptions {
    let ngram_indexing = TextFieldIndexing::default()
        .set_tokenizer("ngram")
        .set_fieldnorms(true)
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    TextOptions::default().set_indexing_options(ngram_indexing)
}

fn ngram_tokenizer() -> TextAnalyzer {
    TextAnalyzer::builder(NgramTokenizer::all_ngrams(2, 3))
        .filter(LowerCaser)
        .filter(AsciiFoldingFilter)
        .build()
}

fn keyword_options() -> TextOptions {
    let keyword_indexing = TextFieldIndexing::default()
        .set_tokenizer("keyword")
        .set_fieldnorms(true)
        .set_index_option(IndexRecordOption::WithFreqs);

    TextOptions::default().set_indexing_options(keyword_indexing)
}

fn keyword_tokenizer() -> TextAnalyzer {
    TextAnalyzer::builder(RawTokenizer::default())
        .filter(LowerCaser)
        .build()
}

pub struct LocationSearch {
    index: Index,
    fields: SearchFields,
    query_parser: QueryParser,
}

struct SearchFields {
    idx: Field,
    name: Field,
    province_or_territory: Field,
    province_or_territory_abbr: Field,
}

impl LocationSearch {
    pub fn new() -> Result<Self> {
        let (index, fields) = Self::configure_search_index()?;
        let query_parser = Self::build_query_parser(&index, &fields);
        Ok(Self {
            index,
            fields,
            query_parser,
        })
    }

    fn configure_search_index() -> Result<(Index, SearchFields)> {
        let mut schema_builder = Schema::builder();

        let fields = SearchFields {
            idx: schema_builder.add_u64_field("idx", NumericOptions::default().set_stored()),
            name: schema_builder.add_text_field("name", ngram_options()),
            province_or_territory: schema_builder
                .add_text_field("province_or_territory", ngram_options()),
            province_or_territory_abbr: schema_builder
                .add_text_field("province_or_territory_abbr", keyword_options()),
        };

        let schema = schema_builder.build();

        let tokenizers = TokenizerManager::new();
        tokenizers.register("ngram", ngram_tokenizer());
        tokenizers.register("keyword", keyword_tokenizer());

        let mut index = Index::create_in_ram(schema);
        index.set_tokenizers(tokenizers);

        Ok((index, fields))
    }

    pub fn index_locations<'a, I: Iterator<Item = (usize, &'a Location)>>(
        &self,
        locations: I,
    ) -> Result<()> {
        let num_threads = std::thread::available_parallelism()?.get();

        let mut writer = self
            .index
            .writer_with_num_threads(num_threads, num_threads * MEM_BYTES_PER_THREAD)?;

        for (idx, location) in locations {
            let mut doc = Document::new();

            doc.add_field_value(self.fields.idx, idx as u64);
            doc.add_field_value(self.fields.name, location.name);
            doc.add_field_value(
                self.fields.province_or_territory,
                location.province_or_territory.to_string(),
            );
            doc.add_field_value(
                self.fields.province_or_territory_abbr,
                location.province_or_territory.to_abbr(),
            );

            writer.add_document(doc)?;
        }

        writer.commit()?;

        Ok(())
    }

    pub fn query(&self, query_text: &str) -> Result<Vec<usize>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        let query = self.query_parser.parse_query(query_text)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let locations = top_docs
            .into_iter()
            .filter_map(|(_score, doc_addr)| {
                let doc = searcher.doc(doc_addr).ok()?;
                let val = doc.get_first(self.fields.idx)?;
                val.as_u64().map(|i| i as usize)
            })
            .collect::<Vec<_>>();

        Ok(locations)
    }

    fn build_query_parser(index: &Index, fields: &SearchFields) -> QueryParser {
        QueryParser::for_index(
            &index,
            vec![
                fields.name,
                fields.province_or_territory,
                fields.province_or_territory_abbr,
            ],
        )
    }
}
