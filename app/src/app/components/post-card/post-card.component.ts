import {Component, OnInit, Input} from '@angular/core';

import {Post} from '../../model/post'
import {TokenStorageService} from "../../service/auth/token-storage.service";
import {PostService} from "../../service/posts/post.service";
import {MatDialog} from "@angular/material/dialog";
import {DeletePostDialogComponent} from "../delete-post-dialog/delete-post-dialog.component";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-post-card',
  templateUrl: './post-card.component.html',
  styleUrls: ['./post-card.component.scss']
})
export class PostCardComponent implements OnInit {

  @Input() index: number = 0;
  @Input() post: Post = {
    id: -1,
    title: '',
    author: '',
    digest: '',
    content: '',
    create_time: '',
    last_update_time: ''
  };

  constructor(
    public tokenService: TokenStorageService,
    private postService: PostService,
    private dialog: MatDialog,
    private snackBar: MatSnackBar

  ) {
  }

  openDialog() {
    const dialogRef = this.dialog.open(DeletePostDialogComponent);

    dialogRef.afterClosed().subscribe(result => {
      console.log(`Dialog result: ${result}`);
    });
  }

  ngOnInit(): void {
  }

  deletePost(id : number) {
    const dialogRef = this.dialog.open(DeletePostDialogComponent);
    dialogRef.afterClosed().subscribe(result => {
      if (result) {
        this.postService.deletePost(id).subscribe(data => {
          if (data.code == 0) {
            this.snackBar.open('删除成功');
            window.location.reload();
          } else {
            this.snackBar.open('删除失败');
          }
        })
      }
    });

  }

}
