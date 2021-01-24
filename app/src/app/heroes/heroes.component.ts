import {Component, OnInit} from '@angular/core';
import {Hero} from '../hero';
// import { HEROES } from "../mock-heroes";
import {HeroService} from '../hero.service';
import {MessageService} from '../message.service';

@Component({
    selector: 'app-heros',
    templateUrl: './heroes.component.html',
    styleUrls: ['./heroes.component.less']
})
export class HeroesComponent implements OnInit {

    heroes: Hero[] = [];

    constructor(private heroService: HeroService, private messageService: MessageService) {
    }

    getHeroes(): void {
        this.heroService.getHeroes()
            .subscribe(heroes => this.heroes = heroes);
    }

    ngOnInit(): void {
        this.getHeroes();
    }

}
