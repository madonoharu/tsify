/* tslint:disable */
/* eslint-disable */
export interface Point {
    x: Coordinate;
    y: Coordinate;
}

export type Coordinate = number;


export function from_js(point: Point): void;

export function into_js(): Point;
