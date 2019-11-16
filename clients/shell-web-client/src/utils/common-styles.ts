import { css } from "lit-element";

export default css`
	* {
		border-radius: var(--border-radius);
		font-family: inherit;
	}

	.primary {
		background-color: var(--first-color);
		color: var(--fourth-color);
	}

	a:link,
	a:visited {
		color: currentColor;
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline dashed;
	}

	a * {
		pointer-events: none;
	}

	a,
	button {
		align-items: center;
		display: flex;
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
