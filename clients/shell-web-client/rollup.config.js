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
		historyApiFallback: "/index.html",
		host: "127.0.0.1",
		port: 10001,
	}),
	production && terser(),
	typescript({
		objectHashIgnoreUnknownHack: true,
	}),
];

export default [
	{
		input: "src/elements/shell-main.ts",
		output: {
			file: "dist/elements/shell-main.js",
			format: "iife",
			name: "ShellModule",
		},
		plugins,
	},
];
