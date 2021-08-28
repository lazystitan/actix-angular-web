import {Injectable} from '@angular/core';
import {MatSnackBar} from '@angular/material/snack-bar';

@Injectable({
  providedIn: 'root'
})
export class NoticeService {

  constructor(
    private snackBar: MatSnackBar
  ) {
  }

  plainNotice(msg: string, duration = 3000): void {
    this.snackBar.open(msg, undefined, {duration});
  }
}
