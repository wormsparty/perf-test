import { Column, Entity, PrimaryGeneratedColumn } from "typeorm";

@Entity("entity")
export class EntityEntity {
    @PrimaryGeneratedColumn('id')
    id: string;

    @Column()
    colonne_1: string;

    @Column()
    colonne_2: string;
}
