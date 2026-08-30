/* tslint:disable */
/* eslint-disable */
export interface Duration {
    secs: number;
}

export interface HoldsDuration {
    d: { secs: number; nanos: number };
}

export interface HoldsGuarded {
    range: { start: string; end: string };
    result: { Ok: number } | { Err: string };
    option: Option<number, string>;
}

export interface HoldsInterval {
    i: Interval;
}

export interface HoldsStdTypes {
    std_duration: { secs: number; nanos: number };
    std_range: { start: number; end: number };
}

export interface Interval {
    secs: number;
}

export interface Option<T, U> {
    a: T;
    b: U;
}

export interface Range<T> {
    label: T;
}

export interface Result<T, E> {
    good: T;
    bad: E;
}
