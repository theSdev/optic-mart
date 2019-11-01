import copy from "rollup-plugin-copy";
import resolve from "rollup-plugin-node-resolve";
import serve from "rollup-plugin-serve";
import { terser } from "rollup-plugin-terser";
import typescript from "rollup-plugin-typescript2";

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
	resolve(),
	serve({
		contentBase: "dist",
		host: "127.0.0.1",
		port: 10001,
	}),
	// terser(),
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
