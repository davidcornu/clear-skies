/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Location } from '../models/Location';
import type { LocationResultsPage } from '../models/LocationResultsPage';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class LocationsService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Listing of all available locations
     * @returns LocationResultsPage successful operation
     * @throws ApiError
     */
    public locations({
        limit,
        pageToken,
    }: {
        /**
         * Maximum number of items returned by a single call
         */
        limit?: number | null,
        /**
         * Token returned by previous call to retrieve the subsequent page
         */
        pageToken?: string | null,
    }): CancelablePromise<LocationResultsPage> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/locations',
            query: {
                'limit': limit,
                'page_token': pageToken,
            },
        });
    }

    /**
     * Fuzzy search all available locations
     * @returns Location successful operation
     * @throws ApiError
     */
    public locationsSearch({
        q,
    }: {
        /**
         * Search query (e.g. "montreal")
         */
        q: string,
    }): CancelablePromise<Array<Location>> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/locations/search',
            query: {
                'q': q,
            },
        });
    }

}
