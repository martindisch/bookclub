export type BookResponse = {
  id: string;
  title: string;
  author: string;
  description: string;
  pageCount: number;
  pitchBy: string;
  firstSuggested: string;
  supporters: string[];
};

export const responseToBook = (response: BookResponse): Book => {
  return {
    ...response,
    firstSuggested: new Date(Date.parse(response.firstSuggested)),
  };
};

export type Book = {
  id: string;
  title: string;
  author: string;
  description: string;
  pageCount: number;
  pitchBy: string;
  firstSuggested: Date;
  supporters: string[];
};
