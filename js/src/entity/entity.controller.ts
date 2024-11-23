import { Body, Controller, Post } from '@nestjs/common';
import { EntityService } from './entity.service';
import { EntityEntity } from './entity.entity';
import { GridRequest } from './dto/grid.request';

@Controller('api/list')
export class EntityController {
  constructor(private entityServices: EntityService) {}

  @Post()
  public postEntity(
    @Body() request: GridRequest,
  ): Promise<{ data: EntityEntity[]; total: number }> {
    return this.entityServices.find(request);
  }
}
