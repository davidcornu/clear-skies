/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class DocumentationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Returns the OpenAPI v3.0.3 specification for this server.
     * @returns any successful operation
     * @throws ApiError
     */
    public openapiSchema(): CancelablePromise<any> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/openapi.json',
        });
    }

}
