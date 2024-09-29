import { Module } from '@nestjs/common';
import { EntityController } from './entity/entity.controller';
import { EntityService } from './entity/entity.service';

@Module({
  controllers: [EntityController],
  providers: [EntityService]
})
export class EntityModule {}
