/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ProvinceOrTerritory } from '../models/ProvinceOrTerritory';
import type { WeatherReport } from '../models/WeatherReport';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class WeatherService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * @returns WeatherReport successful operation
     * @throws ApiError
     */
    public weather({
        provinceOrTerritory,
        slug,
    }: {
        provinceOrTerritory: ProvinceOrTerritory,
        slug: string,
    }): CancelablePromise<WeatherReport> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/weather/{province_or_territory}/{slug}',
            path: {
                'province_or_territory': provinceOrTerritory,
                'slug': slug,
            },
        });
    }

}
