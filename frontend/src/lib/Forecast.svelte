<script lang="ts">
  import type { Forecast } from "../openapi";

  export let forecast: Forecast;
</script>

<div class="forecast--cell">
  <div class="forecast--temperature">
    <strong>{forecast.temperature.degrees_c}°C</strong>
  </div>

  <div class="forecast--condition">{forecast.condition}</div>

  {#if forecast.probability_of_precipitation}
    <div class="forecast--pop">
      ☔️ {forecast.probability_of_precipitation}%
    </div>
  {/if}
</div>

<style>
  .forecast--cell {
    padding: 0.5rem;
    background: white;
    height: 100%;
    border-radius: 8px;
  }

  .forecast--cell > * + * {
    margin-block-start: 0.25rem;
  }
</style>
