/* tslint:disable */
/* eslint-disable */
export interface AbiEnvelope<T> {
    data: T;
}

export interface Arrays {
    small: [number, number];
    big: number[];
}

export interface Defaulted {
    opt: number | null;
    map: Record<string, number>;
    big: number;
}

export interface Envelope<T> {
    data: T;
}

export interface Payload {
    name: string;
}


export function abi_roundtrip(v: AbiEnvelope): AbiEnvelope;

export function arg_array_big(v: Envelope): void;

export function arg_array_small(v: Envelope): void;

export function arg_map(v: Envelope): void;

export function arg_option(v: Envelope): void;

export function arg_result(v: Envelope): void;

export function arg_tuple(v: Envelope): void;

export function arg_u64(v: Envelope): void;

export function arg_vec(v: Envelope): void;

export function generic_arg(v: Envelope): void;

export function generic_builtin(v: Envelope): void;

export function generic_nested(v: Envelope): void;

export function generic_return(v: Envelope): Envelope;

export function plain_arg(v: Payload): void;

export function plain_return(v: Payload): Payload;
