<script lang="ts">
  export let apiURL: string;

  import { onDestroy, onMount } from "svelte";
  import "@trevoreyre/autocomplete-js/dist/style.css";
  import Autocomplete from "@trevoreyre/autocomplete-js";
  import { WeatherClient, type Location, type WeatherReport } from "./openapi";
  import Report from "./lib/Report.svelte";

  let autocomplete: Autocomplete;
  let location: Location;
  let reportPromise: Promise<WeatherReport>;

  const client = new WeatherClient({ BASE: apiURL });

  const fetchResults = (searchQuery: string) => {
    return client.locations.locationsSearch({ q: searchQuery });
  };

  const fetchWeather = (location: Location): Promise<WeatherReport> => {
    return client.weather.weather({
      provinceOrTerritory: location.province_or_territory,
      slug: location.slug,
    });
  };

  $: {
    if (location != null) {
      reportPromise = fetchWeather(location);
    }
  }

  onMount(() => {
    autocomplete = new Autocomplete("#location-autocomplete", {
      search(input) {
        return fetchResults(input);
      },
      getResultValue(item) {
        return item.name;
      },
      debounceTime: 200,
      onSubmit(item) {
        location = item;
      },
      autoSelect: true,
    });
  });

  onDestroy(() => {
    autocomplete.destroy();
  });
</script>

<div class="try">
  <div id="location-autocomplete" class="autocomplete">
    <input class="autocomplete-input" placeholder="Search for a city" />
    <ul class="autocomplete-result-list" />
  </div>

  {#if reportPromise != null}
    {#await reportPromise}
      Loading
    {:then report}
      <Report {report} />
    {/await}
  {/if}
</div>

<style>
  :global(.try > * + *) {
    margin-block-start: 12px;
  }
</style>
