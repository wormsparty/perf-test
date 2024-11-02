import { Column, Entity, PrimaryGeneratedColumn } from 'typeorm';

@Entity('entity')
export class EntityEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  colonne_1: string;

  @Column()
  colonne_2: string;
}
