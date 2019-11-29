import { css } from "lit-element";

export default css`
	* {
		border-radius: var(--border-radius);
		font-family: inherit;
	}

	.outline-primary {
		background-color: var(--fourth-color);
		border: 1px solid var(--first-color);
		color: var(--first-color);
	}

	.primary {
		background-color: var(--first-color);
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
		margin-left: 4px;
	}

	h1 {
		margin: 0;
	}
`;
