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
    <section className="flex justify-center w-full">
      <div className="p-4 flex flex-wrap justify-center">
        {movies.map((movie: TMovie) => (
          <div
            key={movie.id}
            className="flex flex-col items-center bg-neutral-900/50 border border-neutral-900 gap-4 p-3 w-fit rounded-md m-1"
          >
            <Image
              src={movie.poster}
              alt={movie.title}
              width={200}
              height={200}
              className="w-fit rounded"
            />
            <div className="max-w-[310px]">
              <p className="font-semibold text-rose-300 text-xl text-wrap break-words">
                {movie.title}
              </p>
              <p>
                {movie.director}, {movie.year}
              </p>
              <p className="font-[family-name:var(--font-geist-mono)] text-sm text-neutral-500 mt-1 break-words">
                {movie.id}
              </p>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
};

export default App;
