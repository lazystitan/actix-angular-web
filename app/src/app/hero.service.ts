import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';

import {Hero} from "./hero";
import {HEROES} from "./mock-heroes";
import {Observable, of} from "rxjs";
import {MessageService} from './message.service';

@Injectable({
    providedIn: 'root'
})
export class HeroService {

    constructor(
        private http: HttpClient,
        private messageService: MessageService
    ) {
    }

    private log(message: string) {
        this.messageService.add(`HeroService: ${message}`);
    }

    private heroesUrl = 'heroes';  // URL to web api

    getHeroes(): Observable<Hero[]> {
        this.messageService.add('HeroService: fetched heroes');
        return this.http.get<Hero[]>(this.heroesUrl)
    }

    getHero(id: number): Observable<Hero> {
        // TODO: send the message _after_ fetching the hero
        this.messageService.add(`HeroService: fetched hero id=${id}`);
        if (HEROES.find(hero => hero.id === id) !== undefined) {
            return of(HEROES.find(hero => hero.id === id) as Hero);
        } else {
            return of({
                id: -1,
                name: 'not exist'
            });
        }
    }

}
