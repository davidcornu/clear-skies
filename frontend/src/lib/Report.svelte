<script lang="ts">
  import type {
    CurrentConditions,
    Pressure,
    WeatherReport,
    Wind,
  } from "../openapi";
  import Forecast from "./Forecast.svelte";

  export let report: WeatherReport;

  let ccFields: Array<[string, string]> = [];

  type Formatters = {
    [Property in keyof CurrentConditions]?: (
      value: CurrentConditions[Property]
    ) => string;
  };

  const basic = (value: any) => {
    return `${value}`;
  };

  const terminated = (suffix: string) => {
    return (value: any) => {
      return `${value}${suffix}`;
    };
  };

  const temp = terminated("°C");
  const pct = terminated("%");
  const km = terminated(" km");

  const formatters: Formatters = {
    condition: basic,
    temperature_c: temp,
    humidity_pct: pct,
    humidex_c: temp,
    wind_chill_c: temp,
    aqhi: basic,
    dewpoint_c: temp,
    pressure: (value: Pressure) => {
      const tendency = value.tendency ? ` ${value.tendency}` : "";
      return `${value.kpa} kPa${tendency}`;
    },
    visibility_km: km,
    wind: (value: Wind) => {
      let output = "";

      if (value.speed == "calm") {
        output += "Calm";
      } else {
        output += `${value.speed.kph} km/h`;
      }

      if (value.direction) {
        output += ` ${value.direction}`;
      }

      if (value.gust_kph) {
        output += ` (${value.gust_kph} km/h gusts)`;
      }

      return output;
    },
  };

  const buildFields = (report: WeatherReport): Array<[string, string]> => {
    if (report.current_conditions == null) {
      return [];
    }

    const cc = report.current_conditions;

    const results = [];

    for (let field in formatters) {
      let value = cc[field];

      if (value == null) {
        continue;
      }

      let display = formatters[field](value);

      results.push([field.split("_", 2)[0], display]);
    }

    return results;
  };

  const dateFormatter = new Intl.DateTimeFormat("en-CA", {
    month: "short",
    day: "2-digit",
    timeZone: "UTC",
  });

  const weekdayFormatter = new Intl.DateTimeFormat("en-CA", {
    weekday: "short",
    timeZone: "UTC",
  });

  const dayHeader = (
    date: string
  ): { dayOfWeek: string; shortDate: string } => {
    const epochSeconds = Date.parse(date);
    const utcDate = new Date(epochSeconds);

    return {
      dayOfWeek: weekdayFormatter.format(utcDate),
      shortDate: dateFormatter.format(utcDate),
    };
  };

  $: {
    ccFields = buildFields(report);
  }
</script>

{#if report.special_weather_statements.length > 0}
  <h3>Special Weather Statements</h3>
  {#each report.special_weather_statements as sws}
    <div class="special-weather-statement">
      <h4>{sws.title}</h4>
      <p>{sws.summary}</p>
      <a href={sws.url}>{sws.url}</a>
    </div>
  {/each}
{/if}

{#if ccFields.length > 0}
  <h3>Current Conditions</h3>
  <div class="cards">
    {#each ccFields as [field, value]}
      <div class="card">
        <div class="card--heading">
          {field}
        </div>
        <div class="card--body">
          {value}
        </div>
      </div>
    {/each}
  </div>
{/if}

{#if report.weather_forecasts.length > 0}
  <h3>Forecast</h3>

  <div class="forecasts card">
    <div style:grid-column="2" style:grid-row="1" class="forecast--time-of-day">
      Day
    </div>
    <div style:grid-column="3" style:grid-row="1" class="forecast--time-of-day">
      Night
    </div>
    {#each report.weather_forecasts as day, idx}
      {@const row = idx + 2}
      {@const fields = dayHeader(day.date)}
      <div
        style:grid-row={row}
        style:grid-column="1"
        class="forecast--day-of-week"
        title={day.date}
      >
        <div>
          <strong>{fields.dayOfWeek}</strong>
        </div>
        <div>{fields.shortDate}</div>
      </div>
    {/each}

    {#each report.weather_forecasts as day, idx}
      {@const row = idx + 2}
      {#if day.forecast.scope == "detailed"}
        <div style:grid-row={row} style:grid-column="2">
          <Forecast forecast={day.forecast.content.day} />
        </div>
        <div style:grid-row={row} style:grid-column="3">
          <Forecast forecast={day.forecast.content.night} />
        </div>
      {:else if day.forecast.scope == "abridged"}
        <div style:grid-row={row} style:grid-column="2 / span 2">
          <Forecast forecast={day.forecast.content} />
        </div>
      {:else if day.forecast.scope == "night"}
        <div style:grid-row={row} style:grid-column="3">
          <Forecast forecast={day.forecast.content} />
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .cards {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .card {
    border-radius: 8px;
    border-color: rgba(0, 0, 0, 0.12);
    border-width: 1px;
    border-style: solid;
    background-color: #fff;
    outline: none;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.16);
    text-align: center;
    overflow: hidden;
  }

  .card--heading {
    text-transform: uppercase;
    padding: 0.25rem 0.5rem;
    background-color: #f0f9ff;
  }

  .card--body {
    padding: 0.5rem;
  }

  .special-weather-statement {
    background-color: #fef2f2;
    border-radius: 8px;
    border: 1px solid #fecaca;
    padding: 0.5rem 1rem;
  }

  .forecasts {
    text-align: center;
    display: grid;
    grid-template-columns: max-content repeat(2, 1fr);
    grid-gap: 0.5rem;
    grid-auto-rows: max-content;
    grid-template-rows: max-content;
    background-color: #f0f9ff;
    border-radius: 8px;
    padding: 0.5rem;
  }

  .forecast--day-of-week {
    padding-top: 0.5rem;
  }

  .forecast--day-of-week > * + * {
    margin-block-start: 0.25rem;
  }
</style>
