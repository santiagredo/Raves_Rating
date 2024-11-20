import { Artist } from "./artist";
import { Expense } from "./expense";
import { Location } from "./location";
import { Rating } from "./rating";
import { Rave } from "./rave";

export type RaveDetail = {
    rave: Rave;
    location: Location;
    rating: Rating;
    artists: Artist[];
    expenses: Expense[];
};
