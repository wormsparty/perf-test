import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { EntityEntity } from './entity.entity';
import { EntityRequestDto } from './dto/entity-request.dto';
import { Repository, Brackets } from 'typeorm';

const removeAccents = (str: string) => {
  return str.normalize('NFD').replace(/\p{Diacritic}/gu, '');
};

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
    let builder = this.entityRepository.createQueryBuilder('e').where('1 = 1');

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
      const filterUnaccent = removeAccents(filter.filter);

      if (filter.type === 'equals') {
        param[`f${paramNb}`] = filterUnaccent;
        builder = builder.andWhere(`unaccent(${key}) = :f${paramNb}`, param);
      } else if (filter.type === 'notEquals') {
        param[`f${paramNb}`] = filterUnaccent;
        builder = builder.andWhere(`unaccent(${key}) <> :f${paramNb}`, param);
      } else if (filter.type === 'contains') {
        param[`f${paramNb}`] = `%${filterUnaccent}%`;
        builder = builder.andWhere(
          `unaccent(${key}) ilike :f${paramNb}`,
          param,
        );
      } else if (filter.type === 'notContains') {
        param[`f${paramNb}`] = `%${filterUnaccent}%`;
        builder = builder.andWhere(
          `not unaccent(${key}) ilike :f${paramNb}`,
          param,
        );
      } else if (filter.type === 'startsWith') {
        param[`f${paramNb}`] = `${filterUnaccent}%`;
        builder = builder.andWhere(
          `unaccent(${key}) ilike :f${paramNb}`,
          param,
        );
      } else if (filter.type === 'endsWith') {
        param[`f${paramNb}`] = `%${filterUnaccent}`;
        builder = builder.andWhere(
          `unaccent(${key}) ilike :f${paramNb}`,
          param,
        );
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
      const globalSearchUnaccent = removeAccents(request.globalSearch);

      builder = builder.andWhere(
        new Brackets((qb) => {
          qb = qb.where('1 = 0');

          for (const field of globalSearchableFields) {
            qb = qb.orWhere(`unaccent(${field}) ilike :global`, {
              global: `%${globalSearchUnaccent}%`,
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

    return await builder.getManyAndCount().then(([entities, total]) => {
      return {
        data: entities,
        total: total,
      };
    });
  }
}
