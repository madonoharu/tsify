/* tslint:disable */
/* eslint-disable */
/**
 * A reference names its argument, whatever the declaration defaults to.
 */
export interface Holder {
    wrapped: Wrapper<number>;
    outcome: Outcome<number>;
}

declare namespace Outcome {
    export type Done<T = string> = { Done: T };
    export type Failed = { Failed: number };
}

export type Outcome<T = string> = Outcome.Done<T> | Outcome.Failed;

export interface Wrapper<T = number> {
    value: T;
}


export function into_js(): Holder;
