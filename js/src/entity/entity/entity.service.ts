import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { EntityEntity } from '../entity/entity.entity/entity.entity';
import { EntityRequestDto } from '../dto/entity-request.dto/entity-request.dto';
import { Repository } from 'typeorm';

@Injectable()
export class EntityService {

    constructor(@InjectRepository(EntityEntity) private readonly entityRepository: Repository<EntityEntity>) {};

    async find(request: EntityRequestDto): Promise<{ data: EntityEntity[], total: number}>{
    	const globalSearchableFields = [ 'colonne_1', 'colonne_2' ];
    	
        let builder = await this.entityRepository
	    .createQueryBuilder()
	    .addSelect('COUNT(*) OVER ()', 'total');
  
        Object.keys(request.filter).forEach(key => {
            const filter = request.filter[key];
            
            if (filter.filterType !== "text") {
                return Promise.reject("Unsupported filter type")
            }

            // TODO: VERIFY KEY IS A VALID COLOMN NAME
            if (filter.type === "equals") {
                builder = builder.where(`${key} = :filter`, { filter: filter.filter })
            } else if (filter.type === "notEquals") {
                builder = builder.where(`${key} <> :filter`, { filter: filter.filter })
            } else if (filter.type === "contains") {
                builder = builder.where(`position(:filter in ${key}) > 0`, { filter: filter.filter })
            } else if (filter.type === "notContains") {
                builder = builder.where(`position(:filter in ${key}) = 0`, { filter: filter.filter })
            } else if (filter.type === "startsWith") {
                builder = builder.where(`${key} like :filter`, { filter: filter.filter + '%' })
            } else if (filter.type === "endsWith") {
                builder = builder.where(`${key} like :filter`, { filter: '%' + filter.filter })
            } else if (filter.type === "blank") {
                builder = builder.where(`(${key} <> '') IS NOT TRUE`)
            } else if (filter.type === "notBlank") {
                builder = builder.where(`${key} <> ''`)
            } else {
                return Promise.reject("Unsupported type")
            }
        });

        // TODO: Global filter
        /*if (request.globalSearch) {
            builder = builder.WhereGroup(func(q *pg.Query) (*pg.Query, error) {
                for _, field := range globalSearchableFields {
                    q = q.WhereOr("position(? in ?) > 0", request.GlobalSearch, pg.Ident(field))
                }
                return q, nil
            })
        }*/

        // TODO: Sort
        /*if len(request.Sort) > 0 {
	    sort := request.Sort[0]
	    dataset = dataset.Order(fmt.Sprintf("%s %s", toSnakeCase(sort.ColId), sort.Sort))
        }*/

        // Paging
        builder = builder.skip(request.start).take(request.end - request.start);
        
        return await builder.getRawMany().then((entities) => {
            return {
                'data': entities.map(e => {
                	    return { 'id': e.id, 'colonne_1': e.EntityEntity_colonne_1, 'colonne_2': e.EntityEntity_colonne_2 }
                	}),
                'total': entities.length > 0 ? entities[0].total : 0,
            };
        });
        
        // TODO: Retourner le total ?!
    }
}

