import {Component, OnInit} from '@angular/core';

import { POSTS } from '../mock-posts';

@Component({
    selector: 'app-post-list',
    templateUrl: './post-list.component.html',
    styleUrls: ['./post-list.component.less']
})
export class PostListComponent implements OnInit {

    posts = POSTS;

    constructor() {
    }

    ngOnInit(): void {
    }

}
