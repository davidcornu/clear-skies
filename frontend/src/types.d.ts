declare module "@trevoreyre/autocomplete-js" {
  export default class Autocomplete {
    constructor(selector: string, options: AutocompleteOptions);
    destroy();
  }

  export interface AutocompleteOptions<R> {
    search(input: string): R;
    getResultValue(result: R): string;
    debounceTime: number;
    onSubmit(result: R);
    autoSelect: boolean;
  }
}
