import {
  Body,
  Controller,
  HttpException,
  HttpStatus,
  Post,
} from '@nestjs/common';
import { EntityService } from './entity.service';
import { EntityEntity } from './entity.entity';
import { GridRequest } from './dto/grid.request';

@Controller('api/list')
export class EntityController {
  constructor(private entityServices: EntityService) {}

  @Post()
  public async postEntity(
    @Body() request: GridRequest,
  ): Promise<{ data: EntityEntity[]; total: number }> {
    try {
      return await this.entityServices.find(request);
    } catch(error) {
      console.error('Error while fetching entity:', error);

      throw new HttpException(
        {
          statusCode: HttpStatus.INTERNAL_SERVER_ERROR,
          message: 'Internal error',
        },
        HttpStatus.INTERNAL_SERVER_ERROR,
      );
    }
  }
}
