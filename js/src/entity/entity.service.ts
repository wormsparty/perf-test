import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { EntityEntity } from './entity.entity';
import { GridRequest } from './dto/grid.request';
import { Repository } from 'typeorm';
import { filter } from './utils/filter';

@Injectable()
export class EntityService {
  constructor(
    @InjectRepository(EntityEntity)
    private readonly entityRepository: Repository<EntityEntity>,
  ) {}

  async find(
    request: GridRequest,
  ): Promise<{ data: EntityEntity[]; total: number }> {
    const globalSearchableFields = ['colonne_1', 'colonne_2'];

    const entityColumns = this.entityRepository.metadata.ownColumns;
    const builder = this.entityRepository
      .createQueryBuilder('e')
      .where('1 = 1');

    return filter(builder, request, entityColumns, globalSearchableFields);
  }
}
