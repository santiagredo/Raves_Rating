export type RaveArtistRelation = {
    id: number;
    rave_id: number;
    artist_id: number;
};

export function getRaveArtistRelationType(): RaveArtistRelation {
    let raveArtistRelation = {
        id: 0,
        rave_id: 0,
        artist_id: 0,
    };

    return raveArtistRelation;
}
