/* tslint:disable */
/* eslint-disable */
/**
 * A default is rendered like any other type, so the feature moves it.
 */
export interface Rendered<T = number | null, U = Record<string, number>> {
    t: T;
    u: U;
}

/**
 * A reference names its argument, whatever the declaration defaults to.
 */
export interface Holder {
    wrapped: Wrapper<number>;
    outcome: Outcome<number>;
}

/**
 * A type of our own whose TypeScript name the parameter lists below also use.
 */
export interface T {
    z: number;
}

/**
 * Accepted by TypeScript, but `T` there means the parameter and not the
 * interface — the same default reading as a different type.
 */
export interface Earlier<T, U> {
    t: T;
    u: U;
}

/**
 * TypeScript reads the `T` in `U`'s default as the parameter declared after
 * it, and rejects that outright as `TS2744`.
 */
export interface Later<U, T = string> {
    u: U;
    t: T;
}

export interface Error {
    m: string;
}

export interface Wrapper<T = number> {
    value: T;
}

type __OutcomeError = Error;
/**
 * Read from inside the namespace, `Error` would reach the sibling variant. The
 * alias the namespace hoists its other references to is what it resolves to.
 */
declare namespace Outcome {
    export type Done<T = __OutcomeError> = { Done: T };
    export type Error = { Error: string };
}

/**
 * Read from inside the namespace, `Error` would reach the sibling variant. The
 * alias the namespace hoists its other references to is what it resolves to.
 */
export type Outcome<T = Error> = Outcome.Done<T> | Outcome.Error;


export function into_js(): Holder;

export function wrapped(v: Wrapper): void;
