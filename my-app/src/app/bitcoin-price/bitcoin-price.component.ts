import { Component, ChangeDetectorRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-bitcoin-price',
  templateUrl: './bitcoin-price.component.html',
  styleUrls: ['./bitcoin-price.component.css']
})
export class BitcoinPriceComponent {
  price?: string;
  
  constructor(private http: HttpClient,private cdr: ChangeDetectorRef) { 
  }

  ngOnInit() {
    this.http.get('http://localhost:8000/price/BTCUSDT').subscribe((data: any) => {
      
      this.price = data;
      console.log(data);
      this.cdr.detectChanges();
    });
  }
  
}
