/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CardinalDirection } from './CardinalDirection';
import type { WindSpeed } from './WindSpeed';

/**
 * Wind conditions
 */
export type Wind = {
    direction?: CardinalDirection | null;
    gust_kph?: number | null;
    speed: WindSpeed;
};

