/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CanadaTz } from './CanadaTz';

/**
 * A timestamp with an associated timezone
 */
export type LocalDateTime = {
    /**
     * Local ISO 8601 timestamp with a fixed UTC offset
     */
    ts: string;
    /**
     * The timezone name
     */
    tz: CanadaTz;
};

