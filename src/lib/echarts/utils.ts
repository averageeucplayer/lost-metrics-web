export function linearGrayShade(value: number, total: number): string {
	const ratio = value / total;
	const lightness = 90 - ratio * 80;

	return `hsl(0, 0%, ${lightness}%)`;
}

export function grayShade(value: number, total: number): string {
	const ratio = value / total;
	const logScale = Math.log10(ratio * (1 / 0.01)) / Math.log10(100); 
    const lightness = 80 - logScale * 60;

    return `hsl(0, 0%, ${lightness}%)`;
}