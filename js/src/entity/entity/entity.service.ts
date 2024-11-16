import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { EntityEntity } from "./entity.entity/entity.entity";
import { EntityRequestDto } from '../dto/entity-request.dto/entity-request.dto';
import { Repository, Brackets } from 'typeorm';

@Injectable()
export class EntityService {
  constructor(
    @InjectRepository(EntityEntity)
    private readonly entityRepository: Repository<EntityEntity>,
  ) {}

  async find(
    request: EntityRequestDto,
  ): Promise<{ data: EntityEntity[]; total: number }> {
    const globalSearchableFields = ['colonne_1', 'colonne_2'];

    const entityColumns = this.entityRepository.metadata.ownColumns;

    let builder = this.entityRepository
      .createQueryBuilder('e')
      .addSelect('COUNT(*) OVER ()', 'total')
      .where('1 = 1');

    let paramNb = 1;

    Object.keys(request.filter).forEach((key) => {
      const filter = request.filter[key];

      if (filter.filterType !== 'text') {
        return Promise.reject('Unsupported filter type');
      }

      if (!entityColumns.find((column) => column.propertyName === key)) {
        return Promise.reject('Invalid column name');
      }

      const param = {};

      if (filter.type === 'equals') {
        param[`f${paramNb}`] = filter.filter;
        builder = builder.andWhere(`${key} = :f${paramNb}`, param);
      } else if (filter.type === 'notEquals') {
        param[`f${paramNb}`] = filter.filter;
        builder = builder.andWhere(`${key} <> :f${paramNb}`, param);
      } else if (filter.type === 'contains') {
        param[`f${paramNb}`] = `%${filter.filter}%`;
        builder = builder.andWhere(`${key} ilike :f${paramNb}`, param);
      } else if (filter.type === 'notContains') {
        param[`f${paramNb}`] = `%${filter.filter}%`;
        builder = builder.andWhere(`not ${key} ilike :f${paramNb}`, param);
      } else if (filter.type === 'startsWith') {
        param[`f${paramNb}`] = `${filter.filter}%`;
        builder = builder.andWhere(`${key} ilike :f${paramNb}`, param);
      } else if (filter.type === 'endsWith') {
        param[`f${paramNb}`] = `%${filter.filter}`;
        builder = builder.andWhere(`${key} ilike :f${paramNb}`, param);
      } else if (filter.type === 'blank') {
        builder = builder.andWhere(`(${key} <> '') IS NOT TRUE`);
      } else if (filter.type === 'notBlank') {
        builder = builder.andWhere(`${key} <> ''`);
      } else {
        return Promise.reject('Unsupported type');
      }

      paramNb++;
    });

    // Global filter
    if (request.globalSearch) {
      builder = builder.andWhere(
        new Brackets((qb) => {
          qb = qb.where('1 = 0');

          for (const field of globalSearchableFields) {
            qb = qb.orWhere(`${field} ilike :global`, {
              global: `%${request.globalSearch}%`,
            });
          }

          return qb;
        }),
      );
    }

    if (request.sort.length > 0) {
      const sort = request.sort[0];
      const dir = sort.sort.toUpperCase();
      const validDir = ['ASC', 'DESC'];

      if (validDir.indexOf(dir) === -1) {
        return Promise.reject('Invalid sort direction');
      }

      builder = builder.orderBy(sort.colId, dir as 'DESC' | 'ASC');
    }

    // Paging
    builder = builder.skip(request.start).take(request.end - request.start);

    return await builder.getRawMany().then((entities) => {
      return {
        // TODO: Can we do better than manually mapping to entity?
        data: entities.map((e) => {
          return {
            id: e.e_id,
            colonne_1: e.e_colonne_1,
            colonne_2: e.e_colonne_2,
          };
        }),
        total: entities.length > 0 ? parseInt(entities[0].total, 10) : 0,
      };
    });
  }
}
