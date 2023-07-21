/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Forecast } from './Forecast';

export type DailyForecast = ({
    content: Forecast;
    scope: 'night';
} | {
    content: {
        day: Forecast;
        night: Forecast;
    };
    scope: 'detailed';
} | {
    content: Forecast;
    scope: 'abridged';
});

