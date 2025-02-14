import Image from "next/image";
import React from "react";

type TMovie = {
	id: number;
	title: string;
	poster: string;
	year: number;
	director: string;
};

const App: React.FunctionComponent = async () => {
	const response = await fetch("http://127.0.0.1:8000/api/movies");

	const movies: TMovie[] = await response.json();

	return (
		<div className="max-w-lg mx-auto space-y-4 p-4">
			{movies.map((movie: TMovie) => (
				<div
					key={movie.id}
					className="flex flex-row items-center bg-neutral-900/50 border border-neutral-900 gap-4 p-1 rounded m-1"
				>
					<Image
						src={movie.poster}
						alt={movie.title}
						width={100}
						height={100}
					/>
					<div className="mr-4">
						<h2 className="font-[family-name:var(--font-geist-sans)] font-semibold text-rose-300 leading-relaxed text-xl">
							{movie.title}
						</h2>
						<p>
							{movie.director}, {movie.year}
						</p>
						<p className="font-[family-name:var(--font-geist-mono)] text-sm text-neutral-500 mt-1">
							{movie.id}
						</p>
					</div>
				</div>
			))}
		</div>
	);
};

export default App;
