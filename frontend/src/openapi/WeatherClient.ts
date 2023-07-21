/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BaseHttpRequest } from './core/BaseHttpRequest';
import type { OpenAPIConfig } from './core/OpenAPI';
import { FetchHttpRequest } from './core/FetchHttpRequest';

import { DocumentationService } from './services/DocumentationService';
import { LocationsService } from './services/LocationsService';
import { WeatherService } from './services/WeatherService';

type HttpRequestConstructor = new (config: OpenAPIConfig) => BaseHttpRequest;

export class WeatherClient {

    public readonly documentation: DocumentationService;
    public readonly locations: LocationsService;
    public readonly weather: WeatherService;

    public readonly request: BaseHttpRequest;

    constructor(config?: Partial<OpenAPIConfig>, HttpRequest: HttpRequestConstructor = FetchHttpRequest) {
        this.request = new HttpRequest({
            BASE: config?.BASE ?? '',
            VERSION: config?.VERSION ?? '0.1.0',
            WITH_CREDENTIALS: config?.WITH_CREDENTIALS ?? false,
            CREDENTIALS: config?.CREDENTIALS ?? 'include',
            TOKEN: config?.TOKEN,
            USERNAME: config?.USERNAME,
            PASSWORD: config?.PASSWORD,
            HEADERS: config?.HEADERS,
            ENCODE_PATH: config?.ENCODE_PATH,
        });

        this.documentation = new DocumentationService(this.request);
        this.locations = new LocationsService(this.request);
        this.weather = new WeatherService(this.request);
    }
}

