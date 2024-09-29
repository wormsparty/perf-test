import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { EntityEntity } from '../entity/entity.entity/entity.entity';
import { EntityRequestDto } from '../dto/entity-request.dto/entity-request.dto';
import { Repository } from 'typeorm';

@Injectable()
export class EntityService {

    constructor(@InjectRepository(EntityEntity) private readonly entityRepository: Repository<EntityEntity>) {};

    async find(request: EntityRequestDto): Promise<EntityEntity>{
    	return Promise.reject('Not implemented');
        // return await this.userRepository.findOne({where: {uuid: uuid}});
    }

}
