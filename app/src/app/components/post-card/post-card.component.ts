import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';

import {Post} from '../../model/post';
import {TokenStorageService} from '../../service/auth/token-storage.service';
import {PostService} from '../../service/posts/post.service';
import {MatDialog} from '@angular/material/dialog';
import {DeletePostDialogComponent} from '../delete-post-dialog/delete-post-dialog.component';
import {NoticeService} from '../../service/notice/notice.service';

@Component({
  selector: 'app-post-card',
  templateUrl: './post-card.component.html',
  styleUrls: ['./post-card.component.scss']
})
export class PostCardComponent implements OnInit {

  @Input() index = 0;
  @Input() post: Post = {
    id: -1,
    title: '',
    author: '',
    digest: '',
    content: '',
    create_time: '',
    last_update_time: ''
  };

  isLogin = false;

  @Output() deletePostEvent = new EventEmitter<any>();

  constructor(
    private tokenService: TokenStorageService,
    private postService: PostService,
    private dialog: MatDialog,
    private noticeService: NoticeService,
  ) {
  }

  openDialog(): void {
    const dialogRef = this.dialog.open(DeletePostDialogComponent);

    dialogRef.afterClosed().subscribe(result => {
      console.log(`Dialog result: ${result}`);
    });
  }

  ngOnInit(): void {
    this.tokenService.isLogin.subscribe((value) => {
      this.isLogin = value;
    });
  }

  deletePost(id: number): void {
    const dialogRef = this.dialog.open(DeletePostDialogComponent);
    dialogRef.afterClosed().subscribe(result => {
      if (result) {
        this.postService.deletePost(id).subscribe(data => {
          if (data.code === 0) {
            this.noticeService.plainNotice('成功');
            this.deletePostEvent.emit(id);
          } else {
            this.noticeService.plainNotice('失败');
          }
        });
      }
    });
  }

}
