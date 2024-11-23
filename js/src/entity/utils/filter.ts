import { Brackets, SelectQueryBuilder } from 'typeorm';
import { GridRequest } from '../dto/grid.request';
import { ColumnMetadata } from 'typeorm/metadata/ColumnMetadata';
import { plainToClass } from 'class-transformer';
import { EntityEntity } from '../entity.entity';

const removeAccents = (str: string) => {
  return str.normalize('NFD').replace(/\p{Diacritic}/gu, '');
};

const mapRawToEntity = <T>(raw: any[], entity: new () => T): T[] => {
  return raw.map((row) => {
    const entityData = Object.keys(row)
      .filter((key) => key.startsWith('e_'))
      .reduce(
        (acc, key) => {
          const cleanKey = key.replace('e_', '');
          acc[cleanKey] = row[key];
          return acc;
        },
        {} as Record<string, any>,
      );

    return plainToClass(entity, entityData);
  });
};

export const filter = async <T>(
  builder: SelectQueryBuilder<T>,
  request: GridRequest,
  entityColumns: ColumnMetadata[],
  globalSearchableFields: string[],
) => {
  let paramNb = 1;

  Object.keys(request.filter).forEach((key) => {
    const filter = request.filter[key];

    if (filter.filterType !== 'text') {
      throw Error('Unsupported filter type');
    }

    if (!entityColumns.find((column) => column.propertyName === key)) {
      throw Error('Invalid column name');
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
      builder = builder.andWhere(`unaccent(${key}) ilike :f${paramNb}`, param);
    } else if (filter.type === 'notContains') {
      param[`f${paramNb}`] = `%${filterUnaccent}%`;
      builder = builder.andWhere(
        `not unaccent(${key}) ilike :f${paramNb}`,
        param,
      );
    } else if (filter.type === 'startsWith') {
      param[`f${paramNb}`] = `${filterUnaccent}%`;
      builder = builder.andWhere(`unaccent(${key}) ilike :f${paramNb}`, param);
    } else if (filter.type === 'endsWith') {
      param[`f${paramNb}`] = `%${filterUnaccent}`;
      builder = builder.andWhere(`unaccent(${key}) ilike :f${paramNb}`, param);
    } else if (filter.type === 'blank') {
      builder = builder.andWhere(`(${key} <> '') IS NOT TRUE`);
    } else if (filter.type === 'notBlank') {
      builder = builder.andWhere(`${key} <> ''`);
    } else {
      throw Error('Unsupported type');
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
      throw Error('Invalid sort direction');
    }

    builder = builder.orderBy(sort.colId, dir as 'DESC' | 'ASC');
  }

  // Paging
  builder = builder.skip(request.start).take(request.end - request.start);

  return await builder.getRawMany().then((entities) => {
    return {
      data: mapRawToEntity(entities, EntityEntity),
      total: entities.length > 0 ? parseInt(entities[0].total, 10) : 0,
    };
  });
};
