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
		port: 10002,
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
		input: "src/elements/user-list.ts",
		output: {
			file: "dist/elements/user-list.js",
			format: "iife",
			name: "UserList",
		},
		plugins,
	},
	{
		input: "src/elements/user-login.ts",
		output: {
			file: "dist/elements/user-login.js",
			format: "iife",
			name: "UserLogin",
		},
		plugins,
	},
	{
		input: "src/elements/user-register.ts",
		output: {
			file: "dist/elements/user-register.js",
			format: "iife",
			name: "UserRegister",
		},
		plugins,
	},
	{
		input: "src/elements/user-view.ts",
		output: {
			file: "dist/elements/user-view.js",
			format: "iife",
			name: "UserView",
		},
		plugins,
	},
	{
		input: "src/elements/user-search.ts",
		output: {
			file: "dist/elements/user-search.js",
			format: "iife",
			name: "UserSearch",
		},
		plugins,
	},
	{
		input: "src/elements/user-modify.ts",
		output: {
			file: "dist/elements/user-modify.js",
			format: "iife",
			name: "UserModify",
		},
		plugins,
	},
];
