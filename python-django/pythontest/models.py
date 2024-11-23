from django.db import models

class Entity(models.Model):
    id        = models.BigIntegerField(primary_key=True)
    colonne_1 = models.CharField()
    colonne_2 = models.CharField()

    class Meta:
        db_table = "entity"
        managed = False
