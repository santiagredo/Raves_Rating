export type Artist = {
    id: number;
    name: string;
    country: string;
};

export function getArtistType(): Artist {
    let artist = {
        id: 0,
        name: "",
        country: "",
    };

    return artist;
}
