import { Body, Controller, Post } from '@nestjs/common';
import { EntityService } from './entity.service';
import { EntityEntity } from './entity.entity/entity.entity';
import { EntityRequestDto } from '../dto/entity-request.dto/entity-request.dto';

@Controller('api/list')
export class EntityController {
  constructor(private entityServices: EntityService) {}

  @Post()
  public postEntity(
    @Body() request: EntityRequestDto,
  ): Promise<{ data: EntityEntity[]; total: number }> {
    return this.entityServices.find(request);
  }
}
