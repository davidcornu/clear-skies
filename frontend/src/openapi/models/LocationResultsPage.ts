/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Location } from './Location';

/**
 * A single page of results
 */
export type LocationResultsPage = {
    /**
     * list of items on this page of results
     */
    items: Array<Location>;
    /**
     * token used to fetch the next page of results (if any)
     */
    next_page?: string | null;
};

