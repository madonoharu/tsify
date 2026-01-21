/* tslint:disable */
/* eslint-disable */
export interface Point {
    x: number;
    y: number;
}


export function consume(point: Point): void;

export function consume_vector(points: Point[]): void;

export function into_js(): Point;

export function vector_into_js(): Point[];
