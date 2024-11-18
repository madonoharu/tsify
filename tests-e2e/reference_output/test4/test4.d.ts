/* tslint:disable */
/* eslint-disable */
/**
 * @param {Point} point
 */
export function consume(point: Point): void;
/**
 * @returns {Point}
 */
export function into_js(): Point;
/**
 * @param {(Point)[]} points
 */
export function consume_vector(points: (Point)[]): void;
/**
 * @returns {(Point)[]}
 */
export function vector_into_js(): (Point)[];
export interface Point {
    x: number;
    y: number;
}

