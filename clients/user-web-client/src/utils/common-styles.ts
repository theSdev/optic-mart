import { css } from "lit-element";

export default css`
	:host {
		display: block;
	}

	* {
		border-radius: var(--border-radius);
		font-family: inherit;
	}

	.primary,
	[type="submit"] {
		background-color: var(--first-color);
		color: var(--fourth-color);
	}

	.secondary {
		background-color: var(--second-color);
		color: var(--fourth-color);
	}

	a:link,
	a:visited {
		color: currentColor;
		text-decoration: underline solid;
	}

	a * {
		pointer-events: none;
	}

	a,
	button {
		align-items: center;
		display: inline-flex;
		justify-content: center;
	}

	a box-icon,
	button box-icon {
		margin-left: 0.3em;
	}

	article {
		display: grid;
		grid-gap: 1.5em;
		padding: 1em;
	}

	article h1::before {
		content: " ";
		background-color: var(--second-color);
	}

	button {
		padding: 0.5em 1em;
		border: none;
	}

	button:hover {
		filter: contrast(1.1);
	}

	button:active {
		transform: translateX(-1px);
	}

	fieldset {
		border: 1px currentColor solid;
		padding: 1em;
		margin-bottom: 1.5em;
	}

	fieldset > div {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		grid-gap: 1em;
	}

	h1 {
		margin: 0;
	}

	input {
		border: 1px currentColor solid;
		height: 2.5ex;

		padding: 1ex;
	}

	label {
		display: grid;
		grid-gap: 0.5em;
	}
`;
