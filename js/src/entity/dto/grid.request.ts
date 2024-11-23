import { IsString, IsNumber } from 'class-validator';

class Sort {
  @IsString()
  public sort: string;

  @IsString()
  public colId: string;
}

class Filter {
  @IsString()
  public filterType: string;

  @IsString()
  public type: string;

  @IsString()
  public filter: string;
}

export class GridRequest {
  @IsNumber()
  public start: number;

  @IsNumber()
  public end: number;

  public sort: Sort[];

  public filter: Record<string, Filter>;

  @IsString()
  public globalSearch: string;
}
