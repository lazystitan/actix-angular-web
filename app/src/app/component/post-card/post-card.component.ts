import {Component, OnInit, Input} from '@angular/core';

import {Post} from '../../model/post'

@Component({
  selector: 'app-post-card',
  templateUrl: './post-card.component.html',
  styleUrls: ['./post-card.component.less']
})
export class PostCardComponent implements OnInit {

  @Input() post: Post = {
    id: -1,
    title: '',
    author: '',
    content: ``,
    create_time: '',
    last_update_time: ''
  };

  constructor() {
  }

  ngOnInit(): void {

  }

}
