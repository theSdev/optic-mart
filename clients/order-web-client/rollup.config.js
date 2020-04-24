import copy from "rollup-plugin-copy";
import json from "rollup-plugin-json";
import resolve from "rollup-plugin-node-resolve";
import serve from "rollup-plugin-serve";
import { terser } from "rollup-plugin-terser";
import typescript from "rollup-plugin-typescript2";

const production = !process.env.ROLLUP_WATCH;
const plugins = [
	copy({
		targets: [
			{ src: "src/index.html", dest: "dist" },
			{
				src: "src/assets/**/*",
				dest: "dist/assets",
			},
		],
	}),
	json({
		preferConst: true,
	}),
	resolve(),
	serve({
		contentBase: "dist",
		host: "127.0.0.1",
		port: 10004,
		headers: {
			"Access-Control-Allow-Origin": "*",
		},
	}),
	production && terser(),
	typescript({
		objectHashIgnoreUnknownHack: true,
	}),
];

export default [
	{
		input: "src/elements/order-index.ts",
		output: {
			file: "dist/elements/order-index.js",
			format: "iife",
			name: "OrderIndex",
		},
		plugins,
	},
	{
		input: "src/elements/order-place.ts",
		output: {
			file: "dist/elements/order-place.js",
			format: "iife",
			name: "OrderPlace",
		},
		plugins,
	},
];
