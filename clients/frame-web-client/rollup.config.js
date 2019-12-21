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
		port: 10003,
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
		input: "src/elements/frame-create.ts",
		output: {
			file: "dist/elements/frame-create.js",
			format: "iife",
			name: "FrameCreate",
		},
		plugins,
	},
	{
		input: "src/elements/frame-index.ts",
		output: {
			file: "dist/elements/frame-index.js",
			format: "iife",
			name: "FrameIndex",
		},
		plugins,
	},
	{
		input: "src/elements/frame-list.ts",
		output: {
			file: "dist/elements/frame-list.js",
			format: "iife",
			name: "FrameList",
		},
		plugins,
	},
	{
		input: "src/elements/frame-view.ts",
		output: {
			file: "dist/elements/frame-view.js",
			format: "iife",
			name: "FrameView",
		},
		plugins,
	},
];
