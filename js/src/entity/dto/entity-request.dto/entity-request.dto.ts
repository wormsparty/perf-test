import { IsString, IsNumber, MaxLength } from "class-validator";

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

export class EntityRequestDto {
    @IsNumber()
    public start: number;

    @IsNumber()
    public end: string;

    public sort: Sort[];

    public filter: Record<string, Filter>;
    
    @IsString()
    public globalSearch: string;
}

