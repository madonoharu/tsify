/* tslint:disable */
/* eslint-disable */
declare namespace CompoundDefault {
    export type V<T = Record<string, number>> = { V: T };
}

export type CompoundDefault<T> = CompoundDefault.V<T = Record<string, number>>;

declare namespace NoDefault {
    export type V<T> = { V: T };
}

export type NoDefault<T> = NoDefault.V<T>;

declare namespace SimpleDefault {
    export type V<T = string> = { V: T };
}

export type SimpleDefault<T> = SimpleDefault.V<T = string>;

export interface CompoundDefaultStruct<T = Record<string, number>> {
    data: T;
}

export interface MatchingParam<T> {
    data: T;
}

export interface RenamedParam<U> {
    data: T;
}
